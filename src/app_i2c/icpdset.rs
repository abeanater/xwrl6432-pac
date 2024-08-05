#[doc = "Register `ICPDSET` reader"]
pub type R = crate::R<IcpdsetSpec>;
#[doc = "Register `ICPDSET` writer"]
pub type W = crate::W<IcpdsetSpec>;
#[doc = "Field `PDSET0` reader - 0:0\\]
Used to set PDOUT\\[0\\]
bit which corresponds to the SCL GPIO pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[0\\]
bit is set to logic high."]
pub type Pdset0R = crate::BitReader;
#[doc = "Field `PDSET0` writer - 0:0\\]
Used to set PDOUT\\[0\\]
bit which corresponds to the SCL GPIO pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[0\\]
bit is set to logic high."]
pub type Pdset0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDSET1` reader - 1:1\\]
Used to set PDOUT\\[1\\]
bit which corresponds to the SDA GPIO pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[1\\]
bit is set to logic high."]
pub type Pdset1R = crate::BitReader;
#[doc = "Field `PDSET1` writer - 1:1\\]
Used to set PDOUT\\[1\\]
bit which corresponds to the SDA GPIO pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[1\\]
bit is set to logic high."]
pub type Pdset1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:2\\]
Reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:2\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Used to set PDOUT\\[0\\]
bit which corresponds to the SCL GPIO pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[0\\]
bit is set to logic high."]
    #[inline(always)]
    pub fn pdset0(&self) -> Pdset0R {
        Pdset0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Used to set PDOUT\\[1\\]
bit which corresponds to the SDA GPIO pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[1\\]
bit is set to logic high."]
    #[inline(always)]
    pub fn pdset1(&self) -> Pdset1R {
        Pdset1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Used to set PDOUT\\[0\\]
bit which corresponds to the SCL GPIO pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[0\\]
bit is set to logic high."]
    #[inline(always)]
    #[must_use]
    pub fn pdset0(&mut self) -> Pdset0W<IcpdsetSpec> {
        Pdset0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Used to set PDOUT\\[1\\]
bit which corresponds to the SDA GPIO pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[1\\]
bit is set to logic high."]
    #[inline(always)]
    #[must_use]
    pub fn pdset1(&mut self) -> Pdset1W<IcpdsetSpec> {
        Pdset1W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcpdsetSpec> {
        NuW::new(self, 2)
    }
}
#[doc = "I2C Pin Data Set register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpdset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpdset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcpdsetSpec;
impl crate::RegisterSpec for IcpdsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icpdset::R`](R) reader structure"]
impl crate::Readable for IcpdsetSpec {}
#[doc = "`write(|w| ..)` method takes [`icpdset::W`](W) writer structure"]
impl crate::Writable for IcpdsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICPDSET to value 0"]
impl crate::Resettable for IcpdsetSpec {
    const RESET_VALUE: u32 = 0;
}
