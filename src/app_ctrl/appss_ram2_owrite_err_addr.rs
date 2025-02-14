#[doc = "Register `APPSS_RAM2_OWRITE_ERR_ADDR` reader"]
pub type R = crate::R<AppssRam2OwriteErrAddrSpec>;
#[doc = "Register `APPSS_RAM2_OWRITE_ERR_ADDR` writer"]
pub type W = crate::W<AppssRam2OwriteErrAddrSpec>;
#[doc = "Field `address` reader - 31:0\\]
RAM2 ahb2sram write error Address"]
pub type AddressR = crate::FieldReader<u32>;
#[doc = "Field `address` writer - 31:0\\]
RAM2 ahb2sram write error Address"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
RAM2 ahb2sram write error Address"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
RAM2 ahb2sram write error Address"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<AppssRam2OwriteErrAddrSpec> {
        AddressW::new(self, 0)
    }
}
#[doc = "APPSS_RAM2_OWRITE_ERR_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram2_owrite_err_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram2_owrite_err_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssRam2OwriteErrAddrSpec;
impl crate::RegisterSpec for AppssRam2OwriteErrAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_ram2_owrite_err_addr::R`](R) reader structure"]
impl crate::Readable for AppssRam2OwriteErrAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_ram2_owrite_err_addr::W`](W) writer structure"]
impl crate::Writable for AppssRam2OwriteErrAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_RAM2_OWRITE_ERR_ADDR to value 0"]
impl crate::Resettable for AppssRam2OwriteErrAddrSpec {
    const RESET_VALUE: u32 = 0;
}
