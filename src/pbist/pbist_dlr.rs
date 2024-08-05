#[doc = "Register `PBIST_DLR` reader"]
pub type R = crate::R<PbistDlrSpec>;
#[doc = "Register `PBIST_DLR` writer"]
pub type W = crate::W<PbistDlrSpec>;
#[doc = "Field `DLR0` reader - 7:0\\]
Datalogger Register \\[1:0\\]
: Reserved \\[2\\]
: ROM-based testing mode. Setting this bit to 1 enables the PBIST controller to execute test algorithms that are stored in the PBIST ROM \\[3\\]
: Do not change this bit from its default value of 1 \\[4\\]
: Config access mode. Setting this bit allows the host processor to configure the PBIST controller registers \\[7:5\\]
: Reserved"]
pub type Dlr0R = crate::FieldReader;
#[doc = "Field `DLR0` writer - 7:0\\]
Datalogger Register \\[1:0\\]
: Reserved \\[2\\]
: ROM-based testing mode. Setting this bit to 1 enables the PBIST controller to execute test algorithms that are stored in the PBIST ROM \\[3\\]
: Do not change this bit from its default value of 1 \\[4\\]
: Config access mode. Setting this bit allows the host processor to configure the PBIST controller registers \\[7:5\\]
: Reserved"]
pub type Dlr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLR1` reader - 15:8\\]
Datalogger Register \\[8\\]
: Reserevd \\[9\\]
: Default Testing Mode. When in this mode, ROM-based testing is kicked off. If the intention is to perform go/no-go testing via config, write to both this bit and bit \\[2\\]
of the Datalogger Register simultaneously \\[15:10\\]
: Reserevd"]
pub type Dlr1R = crate::FieldReader;
#[doc = "Field `DLR1` writer - 15:8\\]
Datalogger Register \\[8\\]
: Reserevd \\[9\\]
: Default Testing Mode. When in this mode, ROM-based testing is kicked off. If the intention is to perform go/no-go testing via config, write to both this bit and bit \\[2\\]
of the Datalogger Register simultaneously \\[15:10\\]
: Reserevd"]
pub type Dlr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Datalogger Register \\[1:0\\]
: Reserved \\[2\\]
: ROM-based testing mode. Setting this bit to 1 enables the PBIST controller to execute test algorithms that are stored in the PBIST ROM \\[3\\]
: Do not change this bit from its default value of 1 \\[4\\]
: Config access mode. Setting this bit allows the host processor to configure the PBIST controller registers \\[7:5\\]
: Reserved"]
    #[inline(always)]
    pub fn dlr0(&self) -> Dlr0R {
        Dlr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Datalogger Register \\[8\\]
: Reserevd \\[9\\]
: Default Testing Mode. When in this mode, ROM-based testing is kicked off. If the intention is to perform go/no-go testing via config, write to both this bit and bit \\[2\\]
of the Datalogger Register simultaneously \\[15:10\\]
: Reserevd"]
    #[inline(always)]
    pub fn dlr1(&self) -> Dlr1R {
        Dlr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Datalogger Register \\[1:0\\]
: Reserved \\[2\\]
: ROM-based testing mode. Setting this bit to 1 enables the PBIST controller to execute test algorithms that are stored in the PBIST ROM \\[3\\]
: Do not change this bit from its default value of 1 \\[4\\]
: Config access mode. Setting this bit allows the host processor to configure the PBIST controller registers \\[7:5\\]
: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dlr0(&mut self) -> Dlr0W<PbistDlrSpec> {
        Dlr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Datalogger Register \\[8\\]
: Reserevd \\[9\\]
: Default Testing Mode. When in this mode, ROM-based testing is kicked off. If the intention is to perform go/no-go testing via config, write to both this bit and bit \\[2\\]
of the Datalogger Register simultaneously \\[15:10\\]
: Reserevd"]
    #[inline(always)]
    #[must_use]
    pub fn dlr1(&mut self) -> Dlr1W<PbistDlrSpec> {
        Dlr1W::new(self, 8)
    }
}
#[doc = "Datalogger 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_dlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_dlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistDlrSpec;
impl crate::RegisterSpec for PbistDlrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbist_dlr::R`](R) reader structure"]
impl crate::Readable for PbistDlrSpec {}
#[doc = "`write(|w| ..)` method takes [`pbist_dlr::W`](W) writer structure"]
impl crate::Writable for PbistDlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PBIST_DLR to value 0"]
impl crate::Resettable for PbistDlrSpec {
    const RESET_VALUE: u16 = 0;
}
