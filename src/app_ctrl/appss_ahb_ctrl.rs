#[doc = "Register `APPSS_AHB_CTRL` reader"]
pub type R = crate::R<AppssAhbCtrlSpec>;
#[doc = "Register `APPSS_AHB_CTRL` writer"]
pub type W = crate::W<AppssAhbCtrlSpec>;
#[doc = "Field `cpu0_ahb_init` reader - 0:0\\]
Ti internal Register. Modifying this register is not recommended Signal decides whehter ahb interface is enabled or not."]
pub type Cpu0AhbInitR = crate::BitReader;
#[doc = "Field `cpu0_ahb_init` writer - 0:0\\]
Ti internal Register. Modifying this register is not recommended Signal decides whehter ahb interface is enabled or not."]
pub type Cpu0AhbInitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Ti internal Register. Modifying this register is not recommended Signal decides whehter ahb interface is enabled or not."]
    #[inline(always)]
    pub fn cpu0_ahb_init(&self) -> Cpu0AhbInitR {
        Cpu0AhbInitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Ti internal Register. Modifying this register is not recommended Signal decides whehter ahb interface is enabled or not."]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_ahb_init(&mut self) -> Cpu0AhbInitW<AppssAhbCtrlSpec> {
        Cpu0AhbInitW::new(self, 0)
    }
}
#[doc = "APPSS_AHB_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ahb_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ahb_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssAhbCtrlSpec;
impl crate::RegisterSpec for AppssAhbCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_ahb_ctrl::R`](R) reader structure"]
impl crate::Readable for AppssAhbCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_ahb_ctrl::W`](W) writer structure"]
impl crate::Writable for AppssAhbCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_AHB_CTRL to value 0"]
impl crate::Resettable for AppssAhbCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
